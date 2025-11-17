<script setup lang="ts">
definePageMeta({
    layout: "blank"
})
const displaying = ref<boolean>(true);

const scrollHome = () => {
    displaying.value = false;
};
</script>

<template>
    <UMain class="h-dvh w-dvw bg-secondary-purple-600/5 flex justify-center items-center">
        <UContainer class="flex items-center justify-center space-x-3">

            <Transition>
                <h1 v-if="displaying" class="text-4xl text-center">
                    <span>

                        Welcome
                    </span>
                    <UButton @click="scrollHome" icon="lucide:arrow-big-down"
                        class="bg-quinary text-quarternary hover:cursor-pointer p-3 rounded-xl" size="xl">
                    </UButton>
                </h1>
            </Transition>
        </UContainer>

    </UMain>
</template>

<style>
.v-enter-active {
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    transform-origin: top center;
    overflow: hidden;
}

.v-leave-active {
    transition: all 0.15s cubic-bezier(0.4, 0, 0.6, 1);
    transform-origin: top center;
    overflow: hidden;
}

.v-enter-from,
.v-leave-to {
    opacity: 0;
    max-height: 0;
    transform: translateY(-10px);
}

.v-enter-to,
.v-leave-from {
    opacity: 1;
    max-height: 50px;
}
</style>