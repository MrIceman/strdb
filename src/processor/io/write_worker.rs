pub trait WriteWorker {
    fn write(self: &Self, content: &[u8]);
    fn spawn(self: &Self);
}
