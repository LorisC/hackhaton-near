<template>
    <div id="app">
        <p>Welcome on the track trash website</p>
        <p>There is currently {{trackers.length}} trash tracked around the world</p>

    </div>
</template>

<script>


    export default {
        name: 'App',
        data() {
            return {
                trackers: []
            }
        },
        mounted() {
            this.$nextTick().then(async () => {
                this.trackers = await this.$trackerFactoryContract.get_tracker_created();
            })
        }
    }
</script>

<style>
    #app {
        font-family: Avenir, Helvetica, Arial, sans-serif;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        text-align: center;
        color: #2c3e50;
        margin-top: 60px;
    }
</style>
