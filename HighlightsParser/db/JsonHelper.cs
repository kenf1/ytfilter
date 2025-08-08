using System.Text.Json;

public static class JsonHelper
{
    public static void SaveListToJsonFile<T>(IEnumerable<T> dataList, string filePath)
    {
        var jsonOptions = new JsonSerializerOptions
        {
            WriteIndented = true,
            PropertyNamingPolicy = JsonNamingPolicy.CamelCase
        };

        string jsonString = JsonSerializer.Serialize(dataList, jsonOptions);
        File.WriteAllText(filePath, jsonString);
    }
}
