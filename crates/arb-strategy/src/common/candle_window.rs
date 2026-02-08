//! 캔들 데이터 롤링 윈도우.
//!
//! VecDeque 기반으로 최대 window_size개의 f64 값을 유지합니다.
//! 전체 재계산 방식으로 mean/stddev를 계산합니다.

use std::collections::VecDeque;

/// 고정 크기 롤링 윈도우.
///
/// 새 데이터 push 시 윈도우가 가득 차면 가장 오래된 데이터를 자동으로 제거합니다.
#[derive(Debug, Clone)]
pub struct CandleWindow {
    /// 내부 데이터 버퍼
    data: VecDeque<f64>,
    /// 최대 윈도우 크기
    window_size: usize,
}

impl CandleWindow {
    /// 지정된 크기의 새 윈도우를 생성합니다.
    pub fn new(window_size: usize) -> Self {
        Self {
            data: VecDeque::with_capacity(window_size),
            window_size,
        }
    }

    /// 새 값을 윈도우에 추가합니다.
    ///
    /// 윈도우가 가득 차면 가장 오래된 값을 제거합니다.
    pub fn push(&mut self, value: f64) {
        if self.data.len() >= self.window_size {
            self.data.pop_front();
        }
        self.data.push_back(value);
    }

    /// 윈도우가 가득 찼는지 확인합니다 (window_size개 이상).
    pub fn is_ready(&self) -> bool {
        self.data.len() >= self.window_size
    }

    /// 현재 윈도우에 저장된 데이터 개수를 반환합니다.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// 윈도우가 비어있는지 확인합니다.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// 윈도우 크기를 반환합니다.
    pub fn window_size(&self) -> usize {
        self.window_size
    }

    /// 내부 데이터에 대한 참조를 반환합니다.
    pub fn data(&self) -> &VecDeque<f64> {
        &self.data
    }

    /// 가장 최근 값을 반환합니다.
    pub fn last(&self) -> Option<f64> {
        self.data.back().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_window_is_empty() {
        let window = CandleWindow::new(5);
        assert!(window.is_empty());
        assert_eq!(window.len(), 0);
        assert!(!window.is_ready());
        assert_eq!(window.window_size(), 5);
    }

    #[test]
    fn test_push_and_evict() {
        let mut window = CandleWindow::new(3);
        window.push(1.0);
        window.push(2.0);
        window.push(3.0);
        assert!(window.is_ready());
        assert_eq!(window.len(), 3);

        // 윈도우 초과 시 가장 오래된 값 제거
        window.push(4.0);
        assert_eq!(window.len(), 3);
        assert_eq!(window.data()[0], 2.0);
        assert_eq!(window.data()[1], 3.0);
        assert_eq!(window.data()[2], 4.0);
    }

    #[test]
    fn test_not_ready_until_full() {
        let mut window = CandleWindow::new(5);
        window.push(1.0);
        window.push(2.0);
        assert!(!window.is_ready());
        assert_eq!(window.len(), 2);

        window.push(3.0);
        window.push(4.0);
        window.push(5.0);
        assert!(window.is_ready());
    }

    #[test]
    fn test_last() {
        let mut window = CandleWindow::new(3);
        assert_eq!(window.last(), None);
        window.push(1.0);
        assert_eq!(window.last(), Some(1.0));
        window.push(2.0);
        assert_eq!(window.last(), Some(2.0));
    }

    #[test]
    fn test_window_size_one() {
        let mut window = CandleWindow::new(1);
        window.push(10.0);
        assert!(window.is_ready());
        assert_eq!(window.last(), Some(10.0));

        window.push(20.0);
        assert_eq!(window.len(), 1);
        assert_eq!(window.last(), Some(20.0));
    }

    #[test]
    fn test_data_reference() {
        let mut window = CandleWindow::new(3);
        window.push(1.0);
        window.push(2.0);
        let data = window.data();
        assert_eq!(data.len(), 2);
        assert_eq!(data[0], 1.0);
        assert_eq!(data[1], 2.0);
    }
}
