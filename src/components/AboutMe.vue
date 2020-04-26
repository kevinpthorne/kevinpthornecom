<template>
    <div class="container">

        <div class="about-me-container">
            <h1>About me</h1>
        </div>
    </div>
</template>

<script>
    export default {
        name: "AboutMe"
    }
</script>

<style scoped>

    .container {
        /*border: white solid 0.5vw;*/
        background-color: black;
        color: white;

        height: 100vh;
        max-height: 100vh;
        width: 98vw;
        max-width: 98vw;

        display: flex;
        flex-direction: column;
        justify-content: center;
        align-content: center;
    }

    .about-me-container {
        align-self: center;

        display: flex;
        flex-direction: row;
        justify-content: center;
        align-content: center;
    }

    h1, h2, h3, h4, h5, h6, p, span, div {
        font-family: "MS Reference Sans Serif", Arial, sans-serif;
    }

    h1, h2, h3, h4, h5, h6 {
        text-transform: uppercase;
    }

</style>
