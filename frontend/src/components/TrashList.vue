<template>
    <el-card :style="{width: '450px'}">
        <div v-if="trackersArray.length > 0">
            <trash-info-card
                    class="trash_card"
                    v-for="(tracker, index) in trackersArray"
                    :key="tracker.owner + tracker.location + tracker.weight + tracker.trash_type + index"
                    :tracker="tracker"
            />
        </div>
        <div v-else>
            No tracker yet
        </div>
    </el-card>
</template>
<script>
    import TrashInfoCard from "./TrashInfoCard";

    export default {
        name: 'trash-list',
        components: {TrashInfoCard},

        props: {
            trackers: {
                type: Object,
                default() {
                    return {}
                }
            },
        },
        computed: {
            trackersArray() {
                const trackers = [];
                for (let key in this.trackers) {
                    trackers.push(this.trackers[key])
                }
                return trackers;
            }
        },
        mounted() {
            console.log(this.trackers);
        }

    }
</script>
<style>
    .trash_card {
        padding-bottom: 10px;
    }
</style>
