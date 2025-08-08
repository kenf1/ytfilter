public static class ImportEnv
{
    public static void Load(string filePath)
    {
        if (!File.Exists(filePath))
            throw new FileNotFoundException($"The file '{filePath}' does not exist.");

        foreach (var line in File.ReadAllLines(filePath))
        {
            if (string.IsNullOrWhiteSpace(line) || line.StartsWith("#"))
                continue;

            var parts = line.Split('=', 2);
            if (parts.Length != 2)
                continue;

            var key = parts[0].Trim();
            var value = parts[1].Trim();
            Environment.SetEnvironmentVariable(key, value);
        }
    }

    public static string? GetValue(string envKey)
    {
        string? envValue = Environment.GetEnvironmentVariable(envKey);

        if (envValue == null)
        {
            return null;
        }
        else
        {
            return envValue;
        }
    }
}