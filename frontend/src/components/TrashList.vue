<template>
    <el-card>
        <trash-info-card
                v-for="(tracker, index) in trackers"
                :key="tracker.owner + tracker.location + tracker.weight + tracker.trash_type + index"
                :display-price-setting="displayPriceSetting" :tracker="tracker"/>
    </el-card>
</template>
<script>
    import TrashInfoCard from "./TrashInfoCard";

    export default {
        name: 'trash-list',
        components: {TrashInfoCard},
        props: {
            trackers: {
                type: Array
            }
        }
    }
</script>
