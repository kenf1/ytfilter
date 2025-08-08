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

    public static EntryInfo GetEntryData(XElement entry, (XNamespace atom, XNamespace media) ns)
    {
        var entryInfo = new EntryInfo
        {
            Id = entry.Element(ns.atom + "id")?.Value ?? "N/A",
            Published = entry.Element(ns.atom + "published")?.Value ?? "N/A",
            Updated = entry.Element(ns.atom + "updated")?.Value ?? "N/A"
        };

        XElement? mediaGroup = entry.Element(ns.media + "group");
        if (mediaGroup == null)
        {
            Console.WriteLine("No media group found.");
            return entryInfo;
        }

        var mediaGroupInfo = new MediaGroupInfo
        {
            MediaTitle = mediaGroup.Element(ns.media + "title")?.Value ?? "N/A",
            MediaContentUrl = mediaGroup.Element(ns.media + "content")?.Attribute("url")?.Value ?? "N/A",
            MediaDescription = mediaGroup.Element(ns.media + "description")?.Value ?? "N/A"
        };

        mediaGroupInfo.Community = GetMediaCommunity(mediaGroup, ns.media);

        entryInfo.MediaGroup = mediaGroupInfo;
        return entryInfo;
    }

    private static MediaCommunityInfo GetMediaCommunity(XElement mediaGroup, XNamespace mediaNamespace)
    {
        var mediaCommunity = mediaGroup.Element(mediaNamespace + "community");
        if (mediaCommunity == null)
        {
            Console.WriteLine("No community data found.");
            return new MediaCommunityInfo();
        }

        var starRating = mediaCommunity.Element(mediaNamespace + "starRating");
        var statistics = mediaCommunity.Element(mediaNamespace + "statistics");

        return new MediaCommunityInfo
        {
            StarRating = new StarRatingInfo
            {
                Count = starRating?.Attribute("count")?.Value ?? "N/A",
                Average = starRating?.Attribute("average")?.Value ?? "N/A",
                Min = starRating?.Attribute("min")?.Value ?? "N/A",
                Max = starRating?.Attribute("max")?.Value ?? "N/A",
            },
            Statistics = new StatisticsInfo
            {
                Views = statistics?.Attribute("views")?.Value ?? "N/A"
            }
        };
    }
}