Now then.

To be clear, I don't know what you should focus on. Your priorities are your own. That said, I have a few comments.

First, checksums won't stop people from recording communications. They can be used to prevent forgery, though.
Second, must you use MD5? I've heard it's in poor shape these days. (As in, apparently it's been broken in a number of ways.)
Third, any way you can piggyback off of existing things, like, uh, TLS, for secure transport?
impl<Vim>  Rust for Cowboy8625Today at 16:10
https://play.rust-lang.org/?version=nightly&mode=debug&edition=2018&gist=f72829eca35f6cc5e26d08bdf9fd3974
Rust Playground
A browser interface to the Rust compiler to experiment with the language
yeah i dont get it.
tiagoToday at 16:11
afaik you need crypto hashes to make sure data is safe
lenscasToday at 16:12
turns out that it is a big deal. Can't write examples because of that....
Monadic CatToday at 16:13
I don't know what you're trying to do, but any way you can make the user pass in a directory path for you?

lenscasToday at 16:14
I'd rather not. As for what I am trying to do.

I am making a macro that you can pass some teal code to, and it will compile it for you at compile time to lua.
mgostIHToday at 16:14
Use AEAD ciphers and don't do weird stuff like turning off hash verification for performance
Monadic CatToday at 16:14
Ah. Do you just need a known place to dump the output to?
mgostIHToday at 16:14
It'll pretty much never be a bottleneck
lenscasToday at 16:14
no
mgostIHToday at 16:14
if you use TLS you get all of that
lenscasToday at 16:15
I need to make sure that require can still find any teal files in the directory of the crate that is using my macro
for that I need to pass -I {crate_directory_of_user} to  the teal compiler