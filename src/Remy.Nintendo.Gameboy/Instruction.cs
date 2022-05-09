using System.IO.Compression;

namespace Remy.Nintendo.GameBoy;

public record Instruction(
    int Code,
    string Operator,
    IReadOnlyList<Operand> Operands,
    int Bits,
    int Size,
    Time Time,
    FlagEffect Z.
    FlagEffect N,
        Fla);
{
    public int Code { get; init; } = 0;
    public string Operator { get; init; } = "";
    public IReadOnlyList<Operand> Operands { get; init; } = Array.Empty<string>();
    public int Bits { get; init; } = 0;
    public int Size { get; init; } = 0;
    public Time Time { get; init; } = new(0, 0);
    public string Z { get; init; } = "";
    public string N { get; init; } = "";
    public string H { get; init; } = "";
    public string C { get; init; } = "";

    public override string ToString() => $"${Code:X2} {Operator} {string.Join(", ", Operands)} {Bits} {Size} {Time} {Z} {N} {H} {C}";
}
}