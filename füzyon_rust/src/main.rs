// Bir tamsayı dizisinin elemanlarını toplayan bir fonksiyon
fn toplam(dizi: &[i32]) -> i32 {
    let mut toplam = 0; // Toplam değişkeni başlangıçta sıfır olarak tanımlanır.

    for &eleman in dizi {
        toplam += eleman; // Dizideki her eleman toplama eklenir.
    }

    toplam // Toplam değeri fonksiyondan döndürülür.
} 

fn main() {
    let dizi = [1, 2, 3, 4, 5]; // Bir tamsayı dizisi oluşturulur ve değerler atanır.
    let sonuc = toplam(&dizi); // 'toplam' fonksiyonu çağrılarak dizi elemanlarının toplamı hesaplanır.

    println!("Dizi elemanlarının toplamı: {}", sonuc); // Hesaplanan toplam değeri ekrana yazdırılır.
}
