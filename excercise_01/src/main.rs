mod concurrency;
mod generics;

fn main() {
    concurrency::lifetime::lifetime_checker1();
    concurrency::closures::show_hider();
    concurrency::channels::thread_tester()
}
