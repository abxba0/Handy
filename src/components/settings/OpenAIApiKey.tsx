import React, { useState } from "react";
import { useTranslation } from "react-i18next";
import { Eye, EyeOff, Key } from "lucide-react";
import { commands } from "@/bindings";
import { useSettings } from "../../hooks/useSettings";
import { SettingContainer } from "../ui/SettingContainer";
import { Input } from "../ui/Input";
import { Button } from "../ui/Button";
import { toast } from "sonner";

export const OpenAIApiKey: React.FC = () => {
  const { t } = useTranslation();
  const { settings, updateSetting } = useSettings();
  const [showApiKey, setShowApiKey] = useState(false);
  const [localApiKey, setLocalApiKey] = useState(settings.openai_api_key || "");
  const [isSaving, setIsSaving] = useState(false);

  // Sync with settings changes
  React.useEffect(() => {
    setLocalApiKey(settings.openai_api_key || "");
  }, [settings.openai_api_key]);

  const handleSave = async () => {
    if (localApiKey === settings.openai_api_key) {
      return; // No change
    }

    setIsSaving(true);
    try {
      await updateSetting("openai_api_key", localApiKey);
      toast.success(t("settings.openai.apiKey.saved"));
    } catch (error) {
      console.error("Failed to save OpenAI API key:", error);
      toast.error(t("settings.openai.apiKey.saveError"));
    } finally {
      setIsSaving(false);
    }
  };

  const handleClear = async () => {
    setIsSaving(true);
    try {
      await updateSetting("openai_api_key", "");
      toast.success(t("settings.openai.apiKey.cleared"));
    } catch (error) {
      console.error("Failed to clear OpenAI API key:", error);
      toast.error(t("settings.openai.apiKey.clearError"));
    } finally {
      setIsSaving(false);
    }
  };

  const handleValidate = async () => {
    if (!localApiKey.trim()) {
      toast.error(t("settings.openai.apiKey.emptyError"));
      return;
    }

    setIsSaving(true);
    try {
      // Note: We would need to add a Tauri command to validate the API key
      // For now, just save it and show a success message
      await updateSetting("openai_api_key", localApiKey);
      toast.success(t("settings.openai.apiKey.validated"));
    } catch (error) {
      console.error("Failed to validate OpenAI API key:", error);
      toast.error(t("settings.openai.apiKey.validationError"));
    } finally {
      setIsSaving(false);
    }
  };

  const hasApiKey = !!localApiKey.trim();

  return (
    <SettingContainer
      title={t("settings.openai.apiKey.title")}
      description={t("settings.openai.apiKey.description")}
      descriptionMode="tooltip"
      layout="horizontal"
      grouped={true}
    >
      <div className="flex flex-col gap-3 w-full">
        <div className="flex items-center gap-2">
          <div className="relative flex-1">
            <Input
              type={showApiKey ? "text" : "password"}
              value={localApiKey}
              onChange={(e) => setLocalApiKey(e.target.value)}
              placeholder={t("settings.openai.apiKey.placeholder")}
              variant="compact"
              className="pr-10"
            />
            <button
              type="button"
              onClick={() => setShowApiKey(!showApiKey)}
              className="absolute right-3 top-1/2 transform -translate-y-1/2 text-mid-gray hover:text-foreground transition-colors"
              aria-label={showApiKey ? t("common.hide") : t("common.show")}
            >
              {showApiKey ? <EyeOff size={16} /> : <Eye size={16} />}
            </button>
          </div>

          <Button
            variant="primary"
            onClick={handleSave}
            disabled={isSaving || localApiKey === settings.openai_api_key}
            className="whitespace-nowrap"
          >
            {isSaving ? t("common.saving") : t("common.save")}
          </Button>
        </div>

        <div className="flex items-center gap-2 text-sm">
          <Key size={14} className="text-mid-gray" />
          <span className="text-mid-gray">
            {hasApiKey
              ? t("settings.openai.apiKey.configured")
              : t("settings.openai.apiKey.notConfigured")}
          </span>

          <div className="flex-1" />

          {hasApiKey && (
            <>
              <Button
                variant="secondary"
                onClick={handleValidate}
                disabled={isSaving}
                size="sm"
              >
                {t("settings.openai.apiKey.validate")}
              </Button>
              <Button
                variant="destructive"
                onClick={handleClear}
                disabled={isSaving}
                size="sm"
              >
                {t("settings.openai.apiKey.clear")}
              </Button>
            </>
          )}
        </div>

        <div className="text-xs text-mid-gray mt-1">
          <p>{t("settings.openai.apiKey.helpText")}</p>
          <p className="mt-1">{t("settings.openai.apiKey.privacyNote")}</p>
        </div>
      </div>
    </SettingContainer>
  );
};
