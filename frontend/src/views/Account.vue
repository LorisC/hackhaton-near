<template>
    <el-card v-loading="loading">
        <h1>
            {{$wallet.getAccountId()}}
        </h1>
        <div class="main" v-if="registered">
            <trash-list :trackers_id="trackers"
                        :account="new this.$nearAPI.Account(this.$near.connection, this.$wallet.getAccountId())"
                        :loading="loadingTrack"
                        @refresh="get_trackers"
            />
            <track-track-card @track="track"
                              :tracking-trash="trackingTrash"
            >
            </track-track-card>
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
                trackers: [],
                registered: false,
                registering: false,
                loading: false,
                loadingTrack: false,
                trackingTrash: false
            }
        },
        methods: {
            async get_trackers() {
                this.loadingTrack = true;
                this.trackers.length = 0;
                let tracker = await this.$trackerFactoryContract
                    .get_account_tracker({account_id: this.$wallet.getAccountId()});
                console.log(tracker);
                this.trackers.push(...tracker);
                this.loadingTrack = false;
            },
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
        },
        async mounted() {
            this.loading = true;
            await this.$nextTick();
            if (this.$wallet.isSignedIn()) {
                this.$trackerFactoryContract.account = new this.$nearAPI.Account(this.$near.connection, this.$wallet.getAccountId());
                this.$trackerFactoryContract.sender = this.$wallet.getAccountId();
                const registered = await this.$trackerFactoryContract.is_registered({account_id: this.$wallet.getAccountId()});
                this.registered = registered;
                console.log(this.registered, "registered ....");
                this.loading = false;
                if (registered)
                    await this.get_trackers()

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
