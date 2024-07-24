fn main() {
    struct Coin {}

    let a = Coin {};
    let b = a;
    let c = b;

    // 以下はすでに変数の中身を代入して消費しているのでエラーになる
    // let d = a;
}

