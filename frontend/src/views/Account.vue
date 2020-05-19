<template>
    <el-card >
        <h1>
            {{accountId}}
        </h1>
        <div class="main" v-if="registered">
            <trash-list :trackers="trackers"/>
            <track-track-card @track="track"
                              :tracking-trash="trackingTrash"
            />
        </div>
        <register-card v-else :registering="registering" @register="register"></register-card>
    </el-card>
</template>

<script>
    import TrashList from "../components/TrashList";
    import TrackTrackCard from "../components/TrackTrackCard";
    import RegisterCard from "../components/RegisterCard";

    export default {
        name: "Account",
        components: {RegisterCard, TrackTrackCard, TrashList},
        data() {
            return {
                registering: false,
                loadingTrack: false,
                trackingTrash: false
            }
        },
        computed: {
            trackers() {
                let trackers = this.$store.state.tracker_buyOwner[this.$wallet.getAccountId()];
                if (trackers !== undefined)
                    return trackers;
                return {};
            },
            registered() {
                return this.$store.state.registered
            },
            accountId(){
                return this.$wallet.getAccountId()
            }
        },
        methods: {
            async register(registerForm) {
                this.registering = true;

                await this.$trackerFactoryContract.register({
                    address: registerForm.address,
                    contact: registerForm.contact,
                    description: registerForm.description,
                    name: registerForm.name,
                });

                this.registering = false;
                this.registered = true;
            },
            async track(form) {
                this.trackingTrash = true;
                await this.$trackerFactoryContract
                    .create_tracker({
                        trash_type: form.type,
                        location: form.location,
                        weight: form.weight
                    });
                this.trackingTrash = false;
            }
        }
    }
</script>

<style scoped>
    .main {
        display: grid;
        grid-template-columns: auto auto;
    }
</style>
