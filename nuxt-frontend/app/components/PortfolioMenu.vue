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
            label: "Theme",
            icon: "lucide:palette",
            popover: {
                mode: "hover"
            },
            children: [
                {
                    label: "Dark",
                }
                , {
                    label: "Light",
                }
            ]
        }
    ]
]);

const iconSize = 20;
const textSize = "text-sm";
</script>

<template>
    <TransitionGroup>
        <aside class="p-2 h-svh flex flex-col center border-r border-grey-accent-500 items-start"
            :class="!collapsed ? 'w-56' : ''">
            <div class="flex flex-row w-full">
                <button @click="toggleCollapse"
                    class="p-3 rounded-xs flex grow justify-start items-center bg-ui hover:text-secondary hover:bg-muted hover:cursor-pointer space-x-2.5">
                    <Icon :name="collapseButtonIcon" :size="iconSize" />
                    <span v-if="!collapsed" :class="textSize">
                        Collapse
                    </span>
                </button>
            </div>
            <div class="flex flex-col grow justify-between w-full">
                <div v-for="section in items">
                    <NuxtLink v-for="item in section" :class="item.class" active-class="text-info"
                        class="p-3 rounded-xs hover:bg-muted flex grow items-center space-x-2.5" @click="item.onSelect"
                        :to="item.to" :target="item.target">
                        <Icon v-if="item.icon && (typeof item.icon) === 'string'" :name="item.icon" :size="iconSize" />
                        <span v-if="!collapsed && item.label && (typeof item.label) === 'string'" class="text-nowrap"
                            :class="textSize">
                            {{ item.label }}
                        </span>
                    </NuxtLink>
                </div>
            </div>

        </aside>
    </TransitionGroup>
</template>

<style>
.v-enter-active,
.v-leave-active {
    transition: opacity 0.5s ease;
}

.v-enter-from,
.v-leave-to {
    opacity: 0;
}
</style>