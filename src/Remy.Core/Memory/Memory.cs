namespace Remy.Memory;

public abstract class Memory
{
    public abstract Span<byte> Read(int offset, int count);
    public abstract void Write(int offset, Span<byte> content);
}