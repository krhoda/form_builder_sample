## Form Builder Demo:
The Vue based UI is already built, and ready to be served through the Rocket Server
To run the server, first have `cargo` and `rustup` installed, then:
```
$ cd form_builder_example
$ ./run.sh
```
This script moves to the `server` directory, sets the current directory (`spruce_example/server`) to be using the nightly build of Rust, what Rocket.rs requires, then simply runs `cargo run`.

The server will start at `localhost:8000` where you can visit to see the Vue UI.
There are three options along the nav bar, `Form Builder`, `Form Table`, and `Submission Table`.

Visiting the second or third option will initially just show an empty table.

Visiting `Form Builder` will take you to a form which allows you to set the title of the form and add string or number inputs. Inputs require a label, and the form itself requires a title. Inputs can also be removed. No two inputs can have the same label.

Once the form is built, it can be submitted and will be saved by the server as a JSON file in the `schema` folder. After submission, the user is presented with the link to the form. Also, the form is added to the form table screen and can be accessed there.

Visiting a built form, a user can fill out the form and submit it. This submission will be saved in a sub-directory of the `submission` directory. The name of the sub-directory will correspond to the UUID used as the Form ID. The submission will also be visible from the `Submission Table` screen in the regular UI.
