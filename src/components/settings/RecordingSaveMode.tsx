import React from "react";
import { useTranslation } from "react-i18next";
import { Dropdown } from "../ui/Dropdown";
import { SettingContainer } from "../ui/SettingContainer";
import { useSettings } from "../../hooks/useSettings";
import type { RecordingSaveMode } from "@/bindings";

interface RecordingSaveModeProps {
  descriptionMode?: "inline" | "tooltip";
  grouped?: boolean;
}

export const RecordingSaveModeSelector: React.FC<RecordingSaveModeProps> =
  React.memo(({ descriptionMode = "tooltip", grouped = false }) => {
    const { t } = useTranslation();
    const { getSetting, updateSetting, isUpdating } = useSettings();

    const selectedSaveMode = getSetting("recording_save_mode") || "both";

    const handleSaveModeSelect = async (mode: string) => {
      await updateSetting("recording_save_mode", mode as RecordingSaveMode);
    };

    const saveModeOptions = [
      { value: "audio_only", label: t("settings.history.saveMode.audioOnly") },
      { value: "text_only", label: t("settings.history.saveMode.textOnly") },
      { value: "both", label: t("settings.history.saveMode.both") },
    ];

    return (
      <SettingContainer
        title={t("settings.history.saveMode.title")}
        description={t("settings.history.saveMode.description")}
        descriptionMode={descriptionMode}
        grouped={grouped}
      >
        <Dropdown
          options={saveModeOptions}
          selectedValue={selectedSaveMode}
          onSelect={handleSaveModeSelect}
          placeholder={t("settings.history.saveMode.placeholder")}
          disabled={isUpdating("recording_save_mode")}
        />
      </SettingContainer>
    );
  });

RecordingSaveModeSelector.displayName = "RecordingSaveModeSelector";
