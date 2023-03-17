use rust_hash_avatar_fork::Generator;

fn main() {
    Generator::new()
        .create()
        .save_to_png("fractal.png")
        .unwrap();
    Generator::new()
        .set_img_size(250)
        .set_padding(10)
        .set_block_num(8)
        .create()
        .save_to_png("fractal2.png")
        .unwrap();
    let mut avatar = Generator::new();
    avatar.set_img_size(480);
    avatar.set_padding(5);
    avatar.set_block_num(20);
    avatar.create().save_to_png("avatar.png").unwrap();

}
