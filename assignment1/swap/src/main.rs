fn swap(a:&mut i32, b:&mut i32) {
       /*
       * Normalerweise würde die folgenden Anweisung ein Verschieben der Ownership von b und a bedeuten, was nicht möglich wäre, da die Funktion nur Referenzen und keinen Besitz über diese Variablen erhält.
       * Da i32 allerdings Copysemantik anstatt Movesemantik benutzt wird dies hier doch möglich.
        */
       let tmp=*b;
       *b=*a;
       *a=tmp;
}

fn main() {
       let mut a = 1;
       let mut b = 2;
       swap(&mut a, &mut b);
       println!("a: {}, b: {}", a, b);
}
