<template>
    <n-card class="plant-card">
        <template #cover>
            <div class="image-container">
                <n-skeleton style="height: 100%;" />
            </div>
        </template>

        <template #header>
            <n-flex justify="space-between" align="center">
                <n-skeleton />
            </n-flex>
        </template>

        <n-flex>
            <n-space justify="space-between" align="center">
                <n-skeleton />
            </n-space>

            <n-space justify="space-between" align="center">
                <n-skeleton />
            </n-space>
        </n-flex>
    </n-card>
</template>

<style scoped>
.plant-card:hover {
    transition: 0.3s;
    transform: translateY(-5px);
    cursor: pointer;
}

.image-container {
    overflow: hidden;
}

.plant-image {
    width: 100%;
    height: 100%;
    max-height: 100px;
    object-fit: cover;
}

.plant-name {
    font-size: 1.2em;
}
</style>