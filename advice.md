# Failing to connect

In the main.rs file, we can see that they have a default user already assumed to be existing for a quick game, and you have to create that user first. This is why it's unauthorized. email:super@heroes and password:batsignal

# Building with openssl

Quote from https://github.com/sfackler/rust-openssl/issues/1542#issuecomment-954434032

This has to do with the openssl-sys dependency. Looking at their docs was a little more helpful than rust-openssl docs. I had a lot of trouble with getting vcpkg working and cooperating.

What worked for me was using chocolatey to install openssl which installs to C:\Program Files\OpenSSL-Win64 by default. Following the openssl-sys docs knowing this I set some environment variables to match OPENSSL_DIR=C:\Program Files\OpenSSL-Win64 and created a folder called ca to hold the ca certificate from Mozilla that rust-openssl uses from here and set the environment variable to match SSL_CERT_FILE=C:\Program Files\OpenSSL-Win64\ca\cacert.pem.

After this I just had to do cargo build and it worked. VS Code also started highlighting properly again as this also broke that extension for me. Hope this helps.

