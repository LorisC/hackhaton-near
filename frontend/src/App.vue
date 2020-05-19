<template>
    <el-container id="app">
        <el-header>
            <div class="header">
                <h1 class="title" @click="$router.push('/')">No Trash Alliance</h1>
                <div class="login-logout-button">
                    <el-button v-if="!isSignIn" @click="login">Login</el-button>
                    <div v-else class="logout-button">
                        <h2>{{$wallet.getAccountId()}}</h2>
                        <el-button class="button" v-if="isSignIn" @click="logout">Logout</el-button>
                    </div>
                </div>
            </div>
            <el-menu mode="horizontal" :style="{width: '100%'}" :router="true">
                <el-menu-item index="account">Account</el-menu-item>
                <el-menu-item index="market">Market Place</el-menu-item>
            </el-menu>
        </el-header>
        <el-main class="main" v-loading="loading">
            <router-view></router-view>
        </el-main>
    </el-container>
</template>

<script>
    export default {
        name: 'App',
        data() {
            return {
                userinfo: {},
                isSignIn: false,
                contracts: {},
                trackers: {},
                loading: false,
            }
        },
        methods: {
            login() {
                this.$wallet.requestSignIn(this.$contractName, "Track trash",);
                /*  this.$trackerFactoryContract.account = new this.$nearAPI.Account(this.$near.connection, this.$wallet.getAccountId());
                  this.$trackerFactoryContract.sender = this.$wallet.getAccountId()*/
            },
            logout() {
                this.$wallet.signOut();
                window.wallet = this.$wallet;
                this.$router.push("/");
            },
            async fetch_tracker_info() {

                const trackers = await this.$trackerFactoryContract.get_tracker_created();

                const account = new this.$nearAPI.Account(this.$near.connection, this.$wallet.getAccountId());
                for (let tracker_id of trackers) {

                    const contract = new this.$nearAPI.Contract(account, tracker_id, {
                        viewMethods: ['get_owner', "get_location", "get_weight", "get_type",],
                        changeMethods: ['transfer_ownership', 'change_location', "transform", "get_transformation_by_owner", "get_owners"],
                        sender: this.$wallet.getAccountId()
                    });

                    this.contracts[tracker_id] = contract;
                    this.trackers[tracker_id] = {
                        tracker_id,
                        owner: await contract.get_owner(),
                        location: await contract.get_location(),
                        weight: await contract.get_weight(),
                        type: await contract.get_type()
                    }
                }

                this.$store.commit("set_trackers", this.trackers)

            },
            async init(){
                this.loading = true;
                await this.$nextTick();
                if (this.$wallet.isSignedIn()) {
                    this.$trackerFactoryContract.account = new this.$nearAPI.Account(this.$near.connection, this.$wallet.getAccountId());
                    this.$trackerFactoryContract.sender = this.$wallet.getAccountId();
                    this.isSignIn = true;
                    this.$store.commit(
                        'set_registered',
                        await this.$trackerFactoryContract.is_registered({
                            account_id: this.$wallet.getAccountId()
                        })
                    );
                    await this.fetch_tracker_info();

                }
                this.loading = false;
            }

        },
        async mounted() {
           try {
               await this.init()
           }
           catch (e) {
               alert("Something bad happen refresh the page");
               throw e;
           }
        }
    }
</script>

<style scoped>
    #app {
        font-size: 18px;
    }
    .title:hover{
        cursor: pointer;
    }

    .header {
        color: lightslategrey;
        display: grid;
        grid-template-columns: 25% 75%;

        align-content: center;
        justify-content: start;
    }

    .login-logout-button {
        justify-self: end;
        align-self: center;
    }

    .logout-button {
        display: grid;
        grid-template-columns: auto auto;
        column-gap: .2em;
    }

    .button {
        height: 2.7em;
        align-self: center;
        margin: .2em;
    }

    .main {
        margin-top: 3em;

    }
</style>
