pub trait ChainEvent<ChainData> {
    fn new(next_block_number: String, block_number: String, data: ChainData) -> Self;
}
