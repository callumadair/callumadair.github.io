<script setup lang="ts">
import type { NavigationMenuItem } from '@nuxt/ui';
const collapsed = ref(false);
const collapseButtonIcon = ref("lucide:panel-left-close");
const expandButtonClick = () => {
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
      label: "Collapse",
      icon: "lucide:panel-left-close",
      type: "trigger",
      onSelect: expandButtonClick,
      class: "hover:cursor-pointer"
    },
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
    }

  ]
]);

</script>

<template>
  <UApp>
    <div class="flex flex-row">
      <aside>
        <UNavigationMenu class="p-2 h-dvh border-r-[0.25px] border-primary-500" highlight highlight-color="secondary"
          :collapsed=collapsed orientation="vertical" :items="items" />
      </aside>
      <NuxtPage />
      <UFooter class="absolute bottom-0 text-center flex w-full">
        Copyright Callum Adair 2025
        <sup>
          <Icon name="lucide:copyright" />
        </sup>
      </UFooter>
    </div>
  </UApp>
</template>
