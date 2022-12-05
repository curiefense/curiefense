Developing Locally
==================

To do a local build, you will need to install Grunt:

```
npm install -g grunt;
```

Download all dependencies:

```
npm install
```

Then run a build:

```
grunt
```

Auto-rebuild
------------

To automatically re-run the build when a file changes, run:

```
grunt dev
```

Testing on SauceLabs
--------------------

The unit tests can be run in SauceLabs. To do this, you will need to set up
your own SauceLabs account and [set a few environment
variables](http://tiny.cc/z8i48x).

Then, run the `sauce` build target:

```
grunt sauce
```

You can also build individual browsers by name:

```
grunt chrome
```




