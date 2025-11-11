<script setup lang="ts">
import type { NavigationMenuItem } from '@nuxt/ui';

const collapsed = ref<boolean>(false);
const collapseButtonIcon = ref("lucide:panel-left-close");
const toggleCollapse = () => {
    console.log("expanding");
    if (collapsed && collapsed.value === true) {
        collapsed.value = false;
        collapseButtonIcon.value = "lucide:panel-left-close";
    } else {
        collapsed.value = true;
        collapseButtonIcon.value = "lucide:panel-left-open";
    }
};

const items = ref<NavigationMenuItem[][]>([
    [
        {
            label: "Home",
            icon: "lucide:house",
            to: "/"
        },
        {
            label: "Personal Projects",
            icon: "lucide:code",
            to: "/projects/"
        },
        {
            label: "Software I Like",
            icon: "lucide:folder-code",
            to: "/software/",
        },
        {
            label: "Reading List",
            icon: "lucide:book",
            to: "/books/"
        }
    ],
    [
        {
            label: "Github",
            icon: "lucide:github",
            to: "https://github.com/callumadair/callumadair.github.io",
            target: "_blank"
        },
        {
            label: "Gitlab",
            icon: "lucide:gitlab",
            to: "https://gitlab.com/callumadair/callumadair-github-io/-/tree/convert-to-vue-js?ref_type=heads",
            target: "_blank"
        },
        {
            label: "Theme settings",
            icon: "lucide:palette",
        }
    ]
]);
</script>

<template>
    <aside class="h-svh flex flex-col bg-neutral-600 items-center">
        <button @click="toggleCollapse"
            class="flex justify-start items-center p-2 w-full hover:bg-space-blue hover:cursor-pointer space-x-2.5">
            <Icon :name="collapseButtonIcon" />
            <span v-if="!collapsed">
                Collapse
            </span>
        </button>
        <div class="flex flex-col grow justify-between">
            <div v-for="section in items" class="">
                <NuxtLink v-for="item in section" :class="item.class"
                    class="p-2 hover:bg-muted flex flex-row items-center space-x-2.5" @click="item.onSelect"
                    :to="item.to">
                    <Icon v-if="item.icon && (typeof item.icon) === 'string'" :name="item.icon" />
                    <span v-if="!collapsed && item.label && (typeof item.label) === 'string'" class="text-nowrap">
                        {{ item.label }}
                    </span>
                </NuxtLink>
            </div>
        </div>

    </aside>
</template>