use std::path::Path;

fn main() {
    let paths = vec![
        "C:\\Users\\odron\\Documents\\",
        "C:\\Users\\odron\\",
        "C:\\Users\\",
        "C:\\",
    ];

    for p in paths {
        let path = Path::new(p);
        let parent = path.parent();
        println!("Path: '{}', Parent: '{:?}'", p, parent);
    }
}
