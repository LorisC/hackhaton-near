<template>
    <el-container id="app">
        <el-header class="header">
            <h1>No Trash Alliance</h1>
            <div class="button-name">
                <el-button v-if="!isSignIn" @click="login">Login</el-button>
                <div v-else class="button-logout">
                    <h2>{{$wallet.getAccountId()}}</h2>
                    <el-button class="button" v-if="isSignIn" @click="logout">Logout</el-button>
                </div>
            </div>
        </el-header>
        <el-main>
            <div>
                <p>There is currently {{trashTracker.length}} trash tracked around the world</p>
                <p>There is currently {{nb_companies}} companies in the tracking network</p>
            </div>
        </el-main>
    </el-container>
</template>

<script>


    export default {
        name: 'App',
        data() {
            return {
                trashTracker: [],
                userinfo: {},
                nb_companies: 0,
                isSignIn: false
            }
        },
        methods: {
            login() {
                this.$wallet.requestSignIn(this.$contractName, "Track trash",);
                /*  this.$trackerFactoryContract.account = new this.$nearAPI.Account(this.$near.connection, this.$wallet.getAccountId());
                  this.$trackerFactoryContract.sender = this.$wallet.getAccountId()*/
            },
            async track() {
                this.trackingTrash = true;
                console.log(this.trackingForm)
                /* await this.$trackerFactoryContract.create_tracker({
                     type: this.trackingForm.type,
                     location: this.trackingForm.location,
                     weight: this.trackingForm.weight
                 });*/
                this.trackingTrash = false;
            },
            async register() {
                this.registering = true;
                await this.$trackerFactoryContract.register({
                    address: this.registerForm.address,
                    contact: this.registerForm.contact,
                    description: this.registerForm.description,
                    name: this.registerForm.name,
                });
                this.registering = false;
            },
            viewInfo(company) {
                this.selectedCompany = company;
                this.displayInfo = true;
            },
            async loadCompanies() {
                this.loadingCompanies = true;
                this.companies.length = 0;
                const companies = await this.$trackerFactoryContract.get_green_companies();
                console.log(companies)
                this.companies.push(...companies);
                this.loadingCompanies = false;
            },
            logout() {
                this.$wallet.signOut()
                window.wallet = this.$wallet;
            },
            confirmPrice() {
                this.displayPriceSetting = false;
            },
            cancelPrice() {
                this.displayPriceSetting = false;
            },
            confirmTrashEdition() {
            },

            confirmWeightEdition() {
                this.displayEditTrashWeight = false;
            },
            confirmLocationEdition() {
                this.displayEditTrashLocation = false;
            },
            async loadUserInfo() {
                this.userInfo = await this.$trackerFactoryContract.get_company_info({account_id: this.$wallet.getAccountId()})
                this.trashTracker.length = 0;
            },
            async loadTrashTracker() {
                this.trashTracker.push(...(await this.$trackerFactoryContract.get_tracker_created()))
                this.nb_companies = await this.$trackerFactoryContract.get_nb_accounts()
            }
        },
        async mounted() {
            await this.$nextTick();
            if (this.$wallet.isSignedIn()) {
                this.$trackerFactoryContract.account = new this.$nearAPI.Account(this.$near.connection, this.$wallet.getAccountId());
                this.$trackerFactoryContract.sender = this.$wallet.getAccountId();
                this.isSignIn = true;
                this.loadUserInfo();
                await this.loadTrashTracker();
            }
        }
    }
</script>

<style scoped>
    #app {
        font-size: 18px;
    }

    .header {
        color: lightslategrey;
        display: grid;
        grid-template-columns: 25% 75%;
        align-content: center;
        justify-content: start;
    }

    .button-name {
        justify-self: end;
        align-self: center;
    }

    .button-logout {
        display: grid;
        grid-template-columns: auto auto;
        column-gap: .2em;
    }

    .button {
        height: 2.7em;
        align-self: center;
        margin: .2em;
    }

    .travel-map {
        height: 400px;
    }
</style>
