/* global module: false, process: false, __dirname: false, require: false */

// Browsers to test on saucelabs
var browsers = {
    firefox: [
        {
            browserName: 'firefox',
            platform: 'WIN8.1'
        }
    ],
    chrome: [
        {
            browserName: 'chrome',
            platform: 'windows 8.1'
        }
    ],
    ie:[
        {
            browserName: 'internet explorer',
            version: '11',
            platform: 'WIN8.1'
        },
        {
            browserName: 'internet explorer',
            version: '10',
            platform: 'WIN8'
        },
        {
            browserName: 'internet explorer',
            version: '9',
            platform: 'WIN7'
        }
    ],
    android: [
        {
            browserName: 'android',
            platform: 'Linux',
            version: '4.4',
            deviceName: 'Android Emulator'
        },
        {
            browserName: 'android',
            platform: 'Linux',
            version: '5.0',
            deviceName: 'Android Emulator'
        },
        {
            browserName: 'android',
            platform: 'Linux',
            version: '5.1',
            deviceName: 'Android Emulator'
        }
    ],
    ios: [
        {
            browserName: 'iphone',
            platform: 'OS X 10.10',
            version: '9.2',
            deviceName: 'iPhone Simulator'
        }
    ]
};

module.exports = function(grunt) {
    "use strict";

    // Project configuration.
    grunt.initConfig({
        pkg: grunt.file.readJSON('package.json'),

        jshint: {
            files: ['Gruntfile.js', 'src/**/*.js', 'test/**/*.js'],
            options: {
                bitwise: true,
                camelcase: true,
                curly: true,
                eqeqeq: true,
                forin: true,
                immed: true,
                indent: 4,
                latedef: "nofunc",
                newcap: true,
                noarg: true,
                nonew: true,
                noempty: true,
                undef: true,
                unused: true,
                strict: true,
                trailing: true,
                maxlen: 100,
                browser: true,
                globals: {
                    "define": true,
                    "module": true
                }
            }
        },

        uglify: {
            build: {
                src: 'src/picoModal.js',
                dest: 'build/picoModal-<%= pkg.version %>.min.js'
            }
        },

        watch: {
            files: ['src/**/*.js', 'test/**/*.js'],
            tasks: ['default']
        },

        bowerVerify: {
            build: {}
        },

        domTest: {
            files: [ "test/*.js" ]
        },

        // Register saucelabs configuration for each browser group. This allows
        // them to be run individually instead of all or nothing
        'sauce-load': {
            //mockTunnel: true,
            //seleniumHost: 'localhost',
            //seleniumPort: 4444,
            urls: [ 'http://localhost:8080', 'http://localhost:9090' ],
            build: process.env.CI_BUILD_NUMBER || Date.now(),
            name: 'PicoModal unit tests',
            public: "public",
            mode: "followup",
            concurrent: 5,
            browsers: browsers,
            testTimeout: 180000,
            setupTimeout: 180000
        }
    });

    grunt.registerTask('server', function () {
        var app = require('express')();

        app.get('/', function (req, res) {
            res.sendFile(__dirname + "/test/index.html");
        });

        app.get('/picoModal.js', function (req, res) {
            res.sendFile(__dirname + "/src/picoModal.js");
        });

        app.listen(9090);
    });

    // Plugins
    grunt.loadNpmTasks('grunt-contrib-jshint');
    grunt.loadNpmTasks('grunt-contrib-uglify');
    grunt.loadNpmTasks('grunt-contrib-watch');
    grunt.loadNpmTasks('grunt-contrib-connect');
    grunt.loadNpmTasks('grunt-bower-verify');
    grunt.loadNpmTasks('grunt-dom-test');
    grunt.loadNpmTasks('grunt-continue');
    grunt.loadNpmTasks('grunt-sauce-load');

    // Default task(s).
    grunt.registerTask(
        'default',
        ['jshint', 'uglify', 'domTest:test']);

    grunt.registerTask( 'dev', [
        'server', 'domTest:server', 'continue:on',
        'jshint', 'uglify', 'domTest:test',
        'watch']);

    grunt.registerTask(
        'sauce',
        ['default', 'server', 'domTest:server', 'sauce-load']);

    // Register a grunt target for testing individual browser groups
    Object.keys(browsers).forEach(function (browser) {
        grunt.registerTask(
            browser,
            ['default', 'server', 'domTest:server', 'sauce-load:' + browser]);
    });

    // Pull requests don't have access to saucelabs, so we need
    // to disable that
    grunt.registerTask('travis', [
        process.env.TRAVIS_PULL_REQUEST === "false" ?
        'sauce' :
        'default'
    ]);
};

