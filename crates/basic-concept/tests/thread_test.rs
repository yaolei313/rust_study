#[cfg(test)]
mod test {
    use basic_concept::s_thread;

    #[test]
    fn test1() {
        s_thread::study_thread();
    }

    #[test]
    fn test2() {
        s_thread::study_thread_local();
    }

    #[test]
    fn test3() {
        s_thread::study_sync_channel();
    }

    #[test]
    fn test4() {
        s_thread::study_channel2();
    }
}
