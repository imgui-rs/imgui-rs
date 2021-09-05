# Updating Winit

Updating the default version of Winit is very annoying and error prone. We should make some automated way to do it so we don't have any issues in the future, but here is what needs to be done:

1. Make sure that glutin is on the new version of Winit that you want to use. It's easier if our default winit version is on that new default.
2. Update the default in the Cargo.toml by simply changing the default guard.
3. At the top of lib.rs, edit the CFG guards which handle `use winit_x as winit;` such that the new default only relies on a positive feature guard (just copy the form used for the old default). If you don't do this, you'll get some particularly strange errors about two crates being used, since somehow the dependency resolver will pick a winit that the user didn't choose (and presumably won't use if it actually made it to compilation).
4. Profit??
