<template>
    <div class="main">
        <div class="center-container">
            <div class="spacer"></div>
            <div class="menu-container">
                <div class="logo-container">
                    <div class="fading-effect-logo"></div>
                    <img src="../assets/k-logo.svg" class="k-logo-lg" alt="Logo for Kevin Thorne">

                    <span class="logo-name">Kevin P. Thorne</span>
                </div>
                <div class="menu-v-divider slide"></div>
                <div class="menu-items-container">
                    <div class="fading-effect-menu"></div>
                    <div class="menu-items">
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
        border-top: black solid 0.5vw;
        border-bottom: black solid 0.5vw;

        height: 98vh;
        max-height: 98vh;
        width: 99.1vw;
        max-width: 99.1vw;

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

        z-index: 1;
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
        max-width: 0.5vw;
        flex-grow: 1;

        background-color: black;

        margin-right: 2vw;

        z-index: 3;
    }

    .menu-items-container {
        flex-grow: 1;

        z-index: 1;
    }

    .menu-items {
        height: 100%;
        flex-grow: 1;

        display: flex;
        flex-direction: column;
        justify-content: space-around;
    }

    .fading-effect-menu {
        position: absolute;
        top: 1vh;
        right: 1vw;

        height: 55vh;
        max-height: 55vh;
        width: 25vw;
        max-width: 25vw;

        background: white;

        z-index: 2;

        animation: menu-swipe 4s cubic-bezier(.5,.5,0,1);
    }

    .fading-effect-logo {
        position: absolute;
        top: 1vh;
        left: 1vw;

        height: 55vh;
        max-height: 55vh;
        width: 25vw;
        max-width: 25vw;

        background: white;

        z-index: 2;

        animation: logo-swipe 4s cubic-bezier(.5,.5,0,1);
    }

    .slide {
        animation: x-slide 4s cubic-bezier(.5,.5,0,1);
        -webkit-animation: x-slide 4s cubic-bezier(.5,.5,0,1);
        -moz-animation: x-slide 4s cubic-bezier(.5,.5,0,1);
        -o-animation: x-slide 4s cubic-bezier(.5,.5,0,1);
    }

    @keyframes fade-in {
        0% {
            opacity: 0;
        }
        100% {
            opacity: 1;
        }
    }

    @keyframes text-slide {
        0% {
            width: 100%;
        }
        60% {
            width: 100%;
        }
        75% {
            width: 0;
        }
        100% {
            width: 0;
        }
    }

    @keyframes x-slide {
        0% {
            transform: translateX(-5vw) scale(0);
        }
        30% {
            transform: translateX(0) scale(1);
        }
        60% {
            transform: translateX(25vw) scale(1);
        }
        75% {
            transform: translateX(-25vw) scale(1);
        }
        90% {
            transform: translateX(0) scale(1);
        }
        100% {
            transform: translateX(0) scale(1);
        }
    }

    @keyframes menu-swipe {
         0% {
             transform: translateX(-19vw) translateY(22vh);
         }
         30% {
             transform: translateX(-19vw) translateY(22vh);
         }
         60% {
             transform: translateX(5vw) translateY(22vh);
         }
         100% {
         }
     }

    @keyframes logo-swipe {
        0% {
            transform: translateX(29vw) translateY(22vh);
        }
        30% {
            transform: translateX(29vw) translateY(22vh);
        }
        64% {
            transform: translateX(29vw) translateY(22vh);
        }
        72% {
            transform: translateX(0vw) translateY(22vh);
        }
        100% {
        }
    }


</style>
