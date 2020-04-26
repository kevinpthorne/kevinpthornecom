<template>
    <div class="main">
        <div class="center-container">
            <div class="spacer"></div>
            <div class="menu-container">
                <div class="logo-container">
                    <img src="../assets/k-logo.svg" class="k-logo-lg" alt="Logo for Kevin Thorne">

                    <span class="logo-name">Kevin P. Thorne</span>
                </div>
                <div class="menu-v-divider"></div>
                <div class="menu-items-container">
                    <div class="menu-items fade-in">
                        <a>About Me</a>
                        <a>My Services</a>
                        <a>My Work</a>
                        <a>Contact Me</a>
                    </div>
                </div>
            </div>
            <div class="spacer"></div>

        </div>
    </div>
</template>

<script>
    export default {
        name: "Billboard"
    }
</script>

<style scoped>

    .main {
        background: white;
        border: black solid 0.5vw;

        height: 98vh;
        max-height: 98vh;
        width: 98vw;
        max-width: 98vw;

        display: flex;
        flex-direction: column;
        justify-content: center;
        align-content: center;
    }

    .spacer {
        flex-grow: 2;
    }

    .center-container {
        width: 65%;
        height: 65%;

        align-self: center;

        display: flex;
        flex-direction: row;
        justify-content: center;
        align-content: center;
    }

    .logo-container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-content: center;
    }

    .menu-items a {
        font-size: 1.8vw;
        text-transform: uppercase;
    }

    .logo-name, .menu-items a {
        font-family: "MS Reference Sans Serif", Arial, sans-serif;
    }

    .logo-name {
        /*font-variant: small-caps;*/
        align-self: center;
        font-size: 2.25vw;
        margin-bottom: 4vh;
    }

    .menu-container {
        align-self: center;

        display: flex;
        flex-direction: row;
        justify-content: center;
        align-content: center;
    }

    .k-logo-lg {
        width: 50vh;
        height: 150%;
    }

    .menu-v-divider {
        width: 0.5vw;
        flex-grow: 1;

        background-color: black;

        margin-right: 2vw;
    }

    .menu-items-container {
        flex-grow: 1;
    }

    .menu-items {
        height: 100%;
        flex-grow: 1;

        display: flex;
        flex-direction: column;
        justify-content: space-around;
    }

    .fade-in {
        animation: fadeIn ease 5s;
        -webkit-animation: fadeIn ease 5s;
        -moz-animation: fadeIn ease 5s;
        -o-animation: fadeIn ease 5s;
        -ms-animation: fadeIn ease 5s;
    }
    @keyframes fadeIn {
        0% {
            opacity:0;
        }
        100% {
            opacity:1;
        }
    }

    @-moz-keyframes fadeIn {
        0% {
            opacity:0;
        }
        100% {
            opacity:1;
        }
    }

    @-webkit-keyframes fadeIn {
        0% {
            opacity:0;
        }
        100% {
            opacity:1;
        }
    }

    @-o-keyframes fadeIn {
        0% {
            opacity:0;
        }
        100% {
            opacity:1;
        }
    }

    @-ms-keyframes fadeIn {
        0% {
            opacity: 0;
        }
        100% {
            opacity: 1;
        }
    }
</style>
