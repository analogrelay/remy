namespace Remy.Nintendo.GameBoy;

public record struct FlagValues(bool Z, bool H, bool C);
public record struct ArithmeticResult(ushort Result, FlagValues flags);

public static class Arithmetic
{
    public static ArithmeticResult Add16Signed(ushort x, byte y, bool carry)
    {
        throw new NotImplementedException();
    }

    public static ArithmeticResult Add16(ushort x, ushort y, bool carry)
    {
        throw new NotImplementedException();
    }

    public static ArithmeticResult Add8(byte x, byte y, bool carry)
    {
        throw new NotImplementedException();
    }

    public static ArithmeticResult Sub16(ushort x, ushort y, bool carry)
    {
        throw new NotImplementedException();
    }

    public static ArithmeticResult Sub8(byte x, byte y, bool carry)
    {
        throw new NotImplementedException();
    }
}