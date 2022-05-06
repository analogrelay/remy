using Microsoft.CodeAnalysis;
using Microsoft.CodeAnalysis.Text;
using Scriban;
using Scriban.Runtime;
using YamlDotNet.Core;
using YamlDotNet.Serialization;
using YamlDotNet.Serialization.NamingConventions;

namespace Remy.Generators.GameBoy;

[Generator]
public class GameBoyInstructionGenerator: ISourceGenerator
{
    private static readonly DiagnosticDescriptor InvalidYamlWarning = new DiagnosticDescriptor(
        id: "REMY0001",
        title: "Couldn't parse generator input",
        messageFormat: "Couldn't parse generator input: '{0}'",
        category: "Remy",
        DiagnosticSeverity.Warning,
        isEnabledByDefault: true);

    private Template _executor;

    public GameBoyInstructionGenerator()
    {
        _executor = LoadEmbeddedTemplate("Executor.scriban");
    }

    private static Template LoadEmbeddedTemplate(string name)
    {
        var asm = typeof(GameBoyInstructionGenerator).Assembly;
        using var stream = asm.GetManifestResourceStream($"Remy.Generators.Templates.{name}");
        if (stream is null)
        {
            throw new InvalidOperationException($"Could not find input template {name}");
        }
        using var reader = new StreamReader(stream);
        var content = reader.ReadToEnd();
        return Template.Parse(content);
    }

    public void Initialize(GeneratorInitializationContext context)
    {
    }

    public void Execute(GeneratorExecutionContext context)
    {
        var deserializer = new DeserializerBuilder()
            .WithNamingConvention(CamelCaseNamingConvention.Instance)
            .Build();
        
        var files = context.AdditionalFiles.Where(f => f.Path.EndsWith(".gba.yml"));
        var insts = new List<Instruction>();
        foreach (var file in files)
        {
            // Process the file
            var fileInsts = ParseInstructions(context, deserializer, file);
            insts.AddRange(fileInsts);
        }

        insts.Sort((l, r) => l.Code.CompareTo(r.Code));

        var builder = new IndentedStringBuilder();
        builder.AppendLine("// This file generated using Remy.Generators");
        builder.AppendLine("#nullable enable");
        builder.AppendLine();
        builder.AppendLine("namespace Remy.Nintendo.GameBoy;");
        builder.AppendLine();
        builder.AppendLine("public record struct ExecuteResult(int Time, int Size);");
        builder.AppendLine();
        builder.AppendLine("public static class Instructions");
        builder.AppendLine("{");
        using (builder.Indent())
        {
            builder.AppendLine("public delegate ExecuteResult Executor(CpuState cpu, MemoryUnit mmu, ushort arg);");

            foreach (var inst in insts)
            {
                if (inst.Code == 0xCB)
                {
                    continue;
                }

                GenerateFunction(builder, inst);
            }

            builder.AppendLine();
            builder.AppendLine("public static readonly Executor?[] StandardInstructions = new Executor?[]");
            builder.AppendLine("{");
            using (builder.Indent())
            {
                var lastCode = -1;
                foreach (var inst in insts)
                {
                    if (inst.Code is 0xCB or > 0xFF)
                    {
                        continue;
                    }

                    while (lastCode < inst.Code - 1)
                    {
                        builder.AppendLine("null,");
                        lastCode++;
                    }

                    lastCode = inst.Code;
                    builder.AppendLine($"Op{inst.Code:X4}{inst.Operator},");
                }
            }
            builder.AppendLine("};");
            builder.AppendLine();

            builder.AppendLine("public static readonly Executor?[] PrefixedInstructions = new Executor?[]");
            builder.AppendLine("{");
            using (builder.Indent())
            {
                var lastCode = -1;
                foreach (var inst in insts)
                {
                    if ((inst.Code & 0xCB00) == 0)
                    {
                        continue;
                    }

                    var code = inst.Code & 0xFF;

                    while (lastCode < code - 1)
                    {
                        builder.AppendLine("null,");
                        lastCode++;
                    }

                    lastCode = code;
                    builder.AppendLine($"Op{inst.Code:X4}{inst.Operator},");
                }
            }
            builder.AppendLine("};");
        }

        builder.AppendLine("}");

        context.AddSource("GameBoy.Executors.g.cs", builder.ToString());
    }

    private void GenerateFunction(IndentedStringBuilder builder, Instruction inst)
    {
        var model = new ScriptObject();
        model["inst"] = inst;
        var context = new TemplateContext();
        context.PushGlobal(model);

        var content = _executor.Render(context);

        foreach (var line in content.Split(new[] { "\r\n", "\n" }, StringSplitOptions.None))
        {
            builder.AppendLine(line);
        }
    }

    private IList<Instruction> ParseInstructions(GeneratorExecutionContext context, IDeserializer deserializer, AdditionalText file)
    {
        var content = file.GetText()?.ToString();
        if (content is null)
        {
            return new List<Instruction>();
        }

        object? parsed;
        try
        {
            parsed = deserializer.Deserialize(new StringReader(content));
        }
        catch (YamlException yamlEx)
        {
            var diag = Diagnostic.Create(
                InvalidYamlWarning,
                Location.Create(
                    file.Path,
                    TextSpan.FromBounds(yamlEx.Start.Index, yamlEx.End.Index),
                    new LinePositionSpan(
                        new LinePosition(yamlEx.Start.Line, yamlEx.Start.Column),
                        new LinePosition(yamlEx.End.Line, yamlEx.End.Column))),
                yamlEx.InnerException?.Message ?? yamlEx.Message);
            context.ReportDiagnostic(diag);
            return new List<Instruction>();
        }

        var insts = new List<Instruction>();
        if (parsed is null)
        {
            throw new InvalidOperationException("Failed to parse YAML");
        }

        foreach (var item in (IList<object>) parsed)
        {
            static Time ConvertMultiTime(IList<object> v)
            {
                var times = v.Cast<string>().Select(t => int.Parse(t)).ToList();
                var max = times.Max();
                var min = times.Min();
                return new Time(min, max - min);
            }

            var dict = (IDictionary<object, object>) item;
            insts.Add(new Instruction()
            {
                Code = int.Parse((string) dict["code"]),
                Operator = (string) dict["operator"],
                Operands = ((IList<object>) dict["operands"]).Cast<string>().ToList(),
                Bits = int.Parse((string) dict["bits"]),
                Size = int.Parse((string) dict["size"]),
                Time = dict["time"] switch
                {
                    string t => new Time(int.Parse(t), 0),
                    IList<object> l => ConvertMultiTime(l),
                    var x => throw new InvalidOperationException($"The 'type' property had unexpected type: '{x.GetType()}'"),
                },
                Z = (string) dict["z"],
                N = (string) dict["n"],
                H = (string) dict["h"],
                C = (string) dict["c"],
            });
        }

        return insts;
    }
}