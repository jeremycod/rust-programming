use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let text = "Hello, 世界! 😊".to_string();
    println!("Text length is {}", text.len());
    let mut count = 0;
    println!{"Characters"};
    for c in text.chars(){
        count = count + 1;
        println!("{}: {}",count, c);
        
    }
    let mut g_count = 0;
    println!("Graphemes");
    for grapheme in text.graphemes(true){
        g_count = g_count + 1;
        println!("{}:{}", g_count, grapheme);
    }
    let slice1 = &text[0..5];
    let slice2 = &text[0..5];
    let slice3 = &text[0..15];
    println!("Slices: {}, {}, {}", slice1, slice2, slice3);
}