<template>
    <el-card :style="{width: '450px'}" v-loading="loading">
        <i class="el-icon-refresh" @click="$emit('refresh')"></i>
        <div v-if="trackers.length > 0">
            <trash-info-card
                    class="trash_card"
                    v-for="(tracker, index) in trackers"
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
        data(){
            return {
                trackers: [],
                contracts: {},
            };
        },
        props: {
            trackers_id: {
                type: Array,
                default(){
                    return []
                }
            },
            account: {
                type: Object,
                required: true,
            },
            loading: {
                type: Boolean,
                required: true
            }
        },
        methods:{
            async fetch_tracker_info(){
               for (let tracker_id of this.trackers_id){
                   const contract = new this.$nearAPI.Contract(this.account, tracker_id, {
                       viewMethods: ['get_owner', "get_location", "get_weight", "get_type", ],
                       changeMethods: ['transfer_ownership', 'change_location', "transform", "get_transformation_by_owner", "get_owners"],
                       sender: this.$wallet.getAccountId()
                   });

                   this.trackers.push({
                       tracker_id,
                       owner: await contract.get_owner(),
                       location: await contract.get_location(),
                       weight: await contract.get_weight(),
                       type: await contract.get_type()
                   });

                   this.contracts[tracker_id] = contract
               }
            }
        },
        async mounted() {
            console.log(this.trackers_id, 'dasdasdasda');
            await this.fetch_tracker_info();
        }

    }
</script>
<style>
    .trash_card{
        padding-bottom: 10px;
    }
</style>
