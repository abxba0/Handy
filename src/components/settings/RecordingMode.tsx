import React from "react";
import { useTranslation } from "react-i18next";
import { Dropdown } from "../ui/Dropdown";
import { SettingContainer } from "../ui/SettingContainer";
import { useSettings } from "../../hooks/useSettings";

interface RecordingModeProps {
  descriptionMode?: "inline" | "tooltip";
  grouped?: boolean;
}

export const RecordingModeSelector: React.FC<RecordingModeProps> = React.memo(
  ({ descriptionMode = "tooltip", grouped = false }) => {
    const { t } = useTranslation();
    const { getSetting, updateSetting, isUpdating } = useSettings();

    const selectedRecordingMode =
      getSetting("recording_mode") || "push_to_talk";

    const handleRecordingModeSelect = async (mode: string) => {
      await updateSetting("recording_mode", mode as any);
    };

    const recordingModeOptions = [
      { value: "push_to_talk", label: t("settings.recordingMode.pushToTalk") },
      {
        value: "voice_activated",
        label: t("settings.recordingMode.voiceActivated"),
      },
    ];

    return (
      <SettingContainer
        title={t("settings.recordingMode.title")}
        description={t("settings.recordingMode.description")}
        descriptionMode={descriptionMode}
        grouped={grouped}
      >
        <Dropdown
          options={recordingModeOptions}
          selectedValue={selectedRecordingMode}
          onSelect={handleRecordingModeSelect}
          placeholder={t("settings.recordingMode.placeholder")}
          disabled={isUpdating("recording_mode")}
        />
      </SettingContainer>
    );
  },
);

RecordingModeSelector.displayName = "RecordingModeSelector";
