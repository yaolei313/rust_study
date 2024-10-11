#[cfg(test)]
mod test_async {
    use basic_concept::s_advanced::future_executor;
    use basic_concept::s_advanced::s_async;

    #[test]
    fn test1() {
        s_async::study_async();
    }

    #[test]
    fn test2() {
        future_executor::study_executor();
    }
}
