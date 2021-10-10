fn main() {
    // print raw string that includes the double quotes
    println!(
        r###"This raw string started with 'r###"'.
    Therefore it does not end until we reach a quote mark ('"')
    followed immediately by three pound signs ('###'):
    "###
    );

    // String type vs &str
    let noodles = "noodles".to_string();
    let oodles = &noodles[1..];
    let poodles = "❤️";

    println!("poodles: {}", poodles);
    assert_eq!(poodles.len(), 6);
    assert_eq!(poodles.chars().count(), 2);

    assert_eq!(
        format!("{}°{:02}′{:02}″N", 24, 5, 23),
        "24°05′23″N".to_string()
    );

    let bits = vec!["veni", "vidi", "vici"];
    assert_eq!(bits.concat(), "venividivici");
    assert_eq!(bits.join(","), "veni,vidi,vici");
}
