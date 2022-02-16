pub trait MemoryAR {

    fn appendAddressFor(bytes: i128);
    
    fn close();

    fn getAppendOffset() -> i128;

    fn getExtendSegmentSize() -> i128;

    fn jumpTo(offset: i128);

    fn putBin(value: BinarySequence) -> i128;

    fn putBin(from: i128, len: i128) -> i128;

    fn putBlockOfBytes(from: i128, len: i128);

    fn putBool(value: bool);
    // byte = u8
    fn putByte(b: u8);

    fn putChar(value: char);

    fn putDouble(double value: );

    fn putFloat(float value: );

    fn putInt(value: i32);

    fn putLong(value: i128);

    fn putLong128(long l1, long l2);

    fn putLong256(long l0, long l1, long l2, long l3);

    fn putLong256(Long256 value);

    fn putLong256(CharSequence hexString);

    fn putLong256(@NotNull CharSequence hexString, int start, int end);

    fn putNullBin() -> i128;

    fn putNullStr() -> i128;

    fn putShort(short value);

    fn putStr(CharSequence value) -> i128;

    fn putStr(value: char) -> i128;

    fn putStr(value: CharSequence, pos: i32, len: i32) -> i128;

    fn skip(bytes: i128);

    fn truncate();
}