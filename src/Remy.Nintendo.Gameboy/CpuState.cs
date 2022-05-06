namespace Remy.Nintendo.GameBoy;

public enum Flag: byte
{
    Z = 0b1000_0000,
    N = 0b0100_0000,
    H = 0b0010_0000,
    C = 0b0001_0000,
}

public class CpuState
{
    public byte A { get; set; }
    public byte F { get; set; }

    public ushort AF
    {
        get => (ushort) ((A << 8) | F);
        set
        {
            A = (byte) (value >> 8);
            F = (byte) (value & 0xFF);
        }
    }

    public byte B { get; set; }
    public byte C { get; set; }

    public ushort BC
    {
        get => (ushort) ((B << 8) | C);
        set
        {
            B = (byte) (value >> 8);
            C = (byte) (value & 0xFF);
        }
    }

    public byte D { get; set; }
    public byte E { get; set; }

    public ushort DE
    {
        get => (ushort) ((D << 8) | E);
        set
        {
            D = (byte) (value >> 8);
            E = (byte) (value & 0xFF);
        }
    }

    public byte H { get; set; }
    public byte L { get; set; }

    public ushort HL
    {
        get => (ushort) ((H << 8) | L);
        set
        {
            H = (byte) (value >> 8);
            L = (byte) (value & 0xFF);
        }
    }

    public ushort SP { get; set; }
    public ushort PC { get; set; }

    public void SetFlag(Flag flag, bool value)
    {
        throw new NotImplementedException();
    }
}