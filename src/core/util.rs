pub fn BZERO(buffer: &mut [u8], length: isize) {
    for i in 0.. length {
        buffer[i] = 0;
    }
}
