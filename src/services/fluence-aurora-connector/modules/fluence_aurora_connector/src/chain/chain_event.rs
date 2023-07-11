pub trait ChainEvent<ChainData> {
    fn new(block_number: String, data: ChainData) -> Self;
}
