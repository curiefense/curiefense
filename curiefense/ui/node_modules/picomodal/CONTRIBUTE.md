How to Contribute
=================

I welcome pull requests, but the build system can be a bit of a pain to
install if you're not accustomed to Grunt. The following steps should get you
up and running (if you're using Ubuntu):

1. Make sure you have the lastest version of Node.js. On Ubuntu, you might
   need a custom repository:

   ```
   $ sudo add-apt-repository ppa:chris-lea/node.js
   $ sudo apt-get update
   ```

   For me, I then had to uninstall and re-install node:

   ```
   $ sudo add-get remove nodejs
   $ sudo apt-get install nodejs
   ```

2. Now, just hop into source directory and install grunt and all the
   dependencies:

   ```
   $ cd Wherever_You_Cloned_PicoModal
   $ npm install
   $ sudo npm install -g grunt-cli
   ```

3. Finally, you're ready to start a build:

   ```
   $ grunt
   ```

