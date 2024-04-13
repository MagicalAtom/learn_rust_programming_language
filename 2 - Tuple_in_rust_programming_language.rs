برای تعریف یک متغیر با چند مقدار که هر مقدار توانایی دریافت نوع داده مخصوص به خود را دارند از تاپل ها استفاده میکنیم 
به این معنا که اگر نیاز به تعریف یک متغیر داریم اما در این متغیر نیاز داریم چندین داده با نوع داده ای متفاوت تعریف کنیم از این قابلیت راست استفاده میکنیم 
نحوه تعریف به این صورت هست  : 

let tup: (i32,f64,u8) = (500,6.4,1);

ابتدا اینکه در قسمت اول نحوه اعمال دستی داده به متغیر ها را مطالعه کنید 
نوع داده ها در راست هم مطالعه کنید 

در اینجا و حالا بعد از تعریف تاپل مورد نظر اگر نیاز به استفاده از مقادیر این متغیر بود به این صورت عمل میکنیم : 
باید مقادیر رو به صورتی که در زیر تعریف میشه به صورت تکی از داخل تاپل بیرون بکشیم 

let (x,y,z) = tup;

در اصل گفته شد که 3  متغیر به اسم های 
x y z
تغریف شد . این متغیر ها هر کدوم به ترتیب مقادیر داخل تاپل را از آن خود کردند
یعنی 
x = 500;
y = 6.4;
z = 1;
به این صورت حالا میتوانیم از مقادیر اعمال شده بر متغیر ها استفاده کنیم 
fn main(){
  print!("x number = {x}");
}
یا اگر نیاز به دسترسی به مقادیر و خانه های تاپل بدون این روش دارید به راحتی با استفاده از دات (نقطه) میتوانید به خانه های تاپل دسترسی پیدا کنید 
ابتدا داخل یک متغیر جدید مقدار را بیرون میکشیم . 
let number2 = tup.2;
print("{number2}");
