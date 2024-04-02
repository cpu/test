fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn failing() {
        panic!("oh no");
    }
}
