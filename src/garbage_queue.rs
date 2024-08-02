use std::collections::VecDeque;

#[derive(Default, Clone, Debug)]
pub struct GarbageQueue {
    queue: VecDeque<u8>
}

#[allow(dead_code)]
impl GarbageQueue {
    pub fn push(&mut self, n: u8) {
        self.add(n)
    }

    pub fn cancel(&mut self, n: u8) {
        self.remove(n);
    }

    pub fn tank(&mut self) -> Option<Vec<u8>> {
        self.remove(8)
    }

    pub fn amt(&self) -> u8 {
        self.queue.iter().sum()
    }

    pub fn num(&self) -> u8 {
        self.queue.len() as u8
    }

    fn add(&mut self, n: u8) {
        if n == 0 { return; }
        self.queue.push_back(n);
    }

    fn remove(&mut self, n: u8) -> Option<Vec<u8>> {
        if self.queue.is_empty() || n == 0 {
            return None
        } 

        let mut out: Vec<u8> = Vec::new();
        let mut total = 0;

        while !self.queue.is_empty() && total < n {
            let g = self.queue.pop_front()?;

            if total + g > n {
                self.queue.push_front(g - (n - total));
                out.push(n - total);
                break;
            }

            total += g;
            out.push(g);
        }
        Some(out)
    }
}

#[cfg(test)]
mod tests {
    use super::GarbageQueue;


    #[test]
    fn count() {
        let mut q = GarbageQueue::default();
        assert_eq!(q.amt(), 0);
        assert_eq!(q.num(), 0);
        q.add(4);
        assert_eq!(q.amt(), 4);
        assert_eq!(q.num(), 1);
        q.add(7);
        assert_eq!(q.amt(), 11);
        assert_eq!(q.num(), 2);
    }

    #[test]
    fn tank_empty_queue() {
        let mut q = GarbageQueue::default();
        assert_eq!(q.remove(0), None);
        assert_eq!(q.remove(4), None);
    }

    #[test]
    fn tank_nothing() {
        let mut q = GarbageQueue::default();
        q.add(3);
        assert_eq!(q.remove(0), None)
    }

    #[test]
    fn tank_garbage() {
        let mut q = GarbageQueue::default();
        q.add(3);
        assert_eq!(q.remove(3), Some(vec![3]));
        assert_eq!(q.remove(255), None);
    }

    #[test]
    fn tank_split() {
        let mut q = GarbageQueue::default();
        q.add(5);
        assert_eq!(q.remove(3), Some(vec![3]));
        assert_eq!(q.remove(3), Some(vec![2]));
        assert_eq!(q.remove(255), None);
    }

    #[test]
    fn tank_multiple() {
        let mut q = GarbageQueue::default();
        q.add(3);
        q.add(2);
        q.add(1);
        q.add(2);
        q.add(6);
        q.add(2);
        assert_eq!(q.remove(6), Some(vec![3, 2, 1]));
        assert_eq!(q.remove(3), Some(vec![2, 1]));
        assert_eq!(q.remove(4), Some(vec![4]));
        assert_eq!(q.remove(3), Some(vec![1,2]));
        assert_eq!(q.remove(255), None);
    }
}