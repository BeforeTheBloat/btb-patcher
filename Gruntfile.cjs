module.exports = function(grunt) {
    grunt.initConfig({
        grunt_android_emulator: {
            emulators: [{
                id: 'emulator-1',
                create: {
                    '--name': 'testAVD',
                    '--force': '',
                    '--target': 'android-18',
                    '--abi': 'armeabi-v7a'
                },
                start: {
                    '-port': '5556',
                    '-no-audio': ''
                }
            }],
            apks: [{
                id: "apk-1",
                path: "./apks/test.apk",
                activities: [{
                    id: "activity-1",
                    packageName: "org.jboss.aerogear",
                    name: "AeroGearMain"
                }]
            }]
        }
    });

    grunt.loadNpmTasks('grunt-android-emulator');

    // Define the launchApk task
    grunt.registerTask('launchApk', [
        'create-android-emulator:emulator-1',
        'start-android-emulator:emulator-1',
        'install-apk:emulator-1:apk-1',
        'start-activity:emulator-1:apk-1:activity-1'
    ]);

    // Set launchApk as the default task
    grunt.registerTask('default', ['launchApk']);
};
