use lukiwiki_parser::parse;

fn main() {
    println!("=== Plugin Syntax Test ===\n");

    // Test 1: Inline plugin with args and content
    let input1 = "&highlight(yellow){important text};";
    println!("Input 1: {}", input1);
    let output1 = parse(input1);
    println!("Output 1: {}\n", output1);

    // Test 2: Inline plugin with args only (no content)
    let input2 = "&icon(mdi-pencil);";
    println!("Input 2: {}", input2);
    let output2 = parse(input2);
    println!("Output 2: {}\n", output2);

    // Test 3: Inline plugin with no args (no content)
    let input3 = "&br;";
    println!("Input 3: {}", input3);
    let output3 = parse(input3);
    println!("Output 3: {}\n", output3);

    // Test 4: Block plugin with args and content
    let input4 = "@code(rust){{ fn main() {} }}";
    println!("Input 4: {}", input4);
    let output4 = parse(input4);
    println!("Output 4: {}\n", output4);

    // Test 5: Block plugin with no args (括弧必須)
    let input5 = "@toc()";
    println!("Input 5: {}", input5);
    let output5 = parse(input5);
    println!("Output 5: {}\n", output5);

    // Test 6: Block plugin with args only (no content)
    let input6 = "@feed(https://example.com/feed.atom, 10)";
    println!("Input 6: {}", input6);
    let output6 = parse(input6);
    println!("Output 6: {}\n", output6);

    // Test 7: @mention (括弧なし) は無視される
    let input7 = "This is @mention without parens";
    println!("Input 7: {}", input7);
    let output7 = parse(input7);
    println!("Output 7: {}\n", output7);

    // Validation
    println!("=== Validation ===");

    if output1.contains("data-args='[\"yellow\"]'") && output1.contains("important text") {
        println!("✓ Inline plugin with args and content works");
    } else {
        println!("✗ Inline plugin with args and content failed");
    }

    if output2.contains("data-args='[\"mdi-pencil\"]'") && output2.contains("/>") {
        println!("✓ Inline plugin with args only (self-closing) works");
    } else {
        println!("✗ Inline plugin with args only failed");
    }

    if output3.contains("data-args='[]'") && output3.contains("plugin-br") && output3.contains("/>")
    {
        println!("✓ Inline plugin with no args (self-closing) works");
    } else {
        println!("✗ Inline plugin with no args failed");
    }

    if output4.contains("data-args='[\"rust\"]'") && output4.contains("fn main()") {
        println!("✓ Block plugin with args and content works");
    } else {
        println!("✗ Block plugin with args and content failed");
    }

    if output5.contains("data-args='[]'") && output5.contains("plugin-toc") {
        println!("✓ Block plugin with args only (no content) works");
    } else {
        println!("✗ Block plugin with args only failed");
        println!("Output: {}", output6);
    }

    if !output7.contains("plugin-") && output7.contains("@mention") {
        println!("✓ @mention without parens is ignored");
    } else {
        println!("✗ @mention handling failed");
    }
}
