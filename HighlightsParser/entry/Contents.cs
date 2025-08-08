using System.Xml.Linq;

class Contents
{
    //todo: get all entries
    public static XElement? GetFirstEntry(XDocument doc, XNamespace atomNamespace)
    {
        return doc.Root?.Element(atomNamespace + "entry");
    }

    public static bool CheckEntryEmpty(XElement? entry)
    {
        if (entry != null)
        {
            return true;
        }
        else
        {
            Console.WriteLine("No entries found in XML.");
            return false;
        }
    }

    public static void DisplayEntryDetails(XElement entry, (XNamespace atom, XNamespace media) ns)
    {
        string id = entry.Element(ns.atom + "id")?.Value ?? "N/A";
        string published = entry.Element(ns.atom + "published")?.Value ?? "N/A";
        string updated = entry.Element(ns.atom + "updated")?.Value ?? "N/A";

        XElement? mediaGroup = entry.Element(ns.media + "group");
        if (mediaGroup == null)
        {
            Console.WriteLine("No media group found.");
            return;
        }

        string mediaTitle = mediaGroup.Element(ns.media + "title")?.Value ?? "N/A";
        string mediaContentUrl = mediaGroup.Element(ns.media + "content")?.Attribute("url")?.Value ?? "N/A";
        string mediaDescription = mediaGroup.Element(ns.media + "description")?.Value ?? "N/A";

        Console.WriteLine("ID: " + id);
        Console.WriteLine("Published: " + published);
        Console.WriteLine("Updated: " + updated);
        Console.WriteLine("Media Title: " + mediaTitle);
        Console.WriteLine("Media Content URL: " + mediaContentUrl);
        Console.WriteLine("Media Description: " + mediaDescription);

        DisplayMediaCommunityDetails(mediaGroup, ns.media);
    }

    public static void DisplayMediaCommunityDetails(XElement mediaGroup, XNamespace mediaNamespace)
    {
        var mediaCommunity = mediaGroup.Element(mediaNamespace + "community");
        if (mediaCommunity == null)
        {
            Console.WriteLine("No community data found.");
            return;
        }

        var starRating = mediaCommunity.Element(mediaNamespace + "starRating");
        string count = starRating?.Attribute("count")?.Value ?? "N/A";
        string average = starRating?.Attribute("average")?.Value ?? "N/A";
        string min = starRating?.Attribute("min")?.Value ?? "N/A";
        string max = starRating?.Attribute("max")?.Value ?? "N/A";

        var statistics = mediaCommunity.Element(mediaNamespace + "statistics");
        string views = statistics?.Attribute("views")?.Value ?? "N/A";

        Console.WriteLine("Star Rating:");
        Console.WriteLine("Count: " + count);
        Console.WriteLine("Average: " + average);
        Console.WriteLine("Min: " + min);
        Console.WriteLine("Max: " + max);
        Console.WriteLine("Views: " + views);
    }
}