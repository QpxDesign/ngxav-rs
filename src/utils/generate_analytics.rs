/*
from parse_line import parse_line
from parse_nginx_time_format import parse_nginx_time_format
from format_file_size import format_file_size


def generate_analytical_output(log_selection):
    stats = {
            "request_count":0,
            "top_5_requests":{},
            "top_5_hosts":{},
            "top_5_ips":{},
            "average_body_byte_speed":0,
            "average_requests_per_minute":0,
            "total_data_transfered":0
            }
    for line in log_selection:
        parsed_line = parse_line(line)
        stats["request_count"] += 1
        try:
            stats["average_body_byte_speed"] += (float(parsed_line["body_bytes_sent"])/(float(parsed_line["request_time"])+.00001))
            stats["total_data_transfered"] += float(parsed_line["body_bytes_sent"])
        except:
            stats["average_body_byte_speed"] += 0

        if parsed_line["request"] not in stats["top_5_requests"]:
            stats["top_5_requests"][parsed_line["request"]] = {
                    "request_text":parsed_line["request"],
                    "count": 0
                    }
        stats["top_5_requests"][parsed_line["request"]]["count"] += 1

        if parsed_line["host"] not in stats["top_5_hosts"]:
            stats["top_5_hosts"][parsed_line["host"]] = {
                    "host_text":parsed_line["host"],
                    "count": 0
                    }

        if parsed_line["ip_address"] not in stats["top_5_ips"]:
            stats["top_5_ips"][parsed_line["ip_address"]] = {
                    "ip_text":parsed_line["ip_address"],
                    "count":0
                    }

        stats["top_5_ips"][parsed_line["ip_address"]]["count"] += 1
        stats["top_5_hosts"][parsed_line["host"]]["count"] += 1

    stats["average_body_byte_speed"] = stats["average_body_byte_speed"] / stats["request_count"]
    stats["average_requests_per_minute"] = stats["request_count"]/((parse_nginx_time_format(parse_line(log_selection[-1])["time"]) - parse_nginx_time_format(parse_line(log_selection[0])["time"])).total_seconds()/60)

    new_requests = []
    new_hosts = []
    new_ips = []
    for request,entry in stats["top_5_requests"].items():
        new_requests.append(entry)

    for host,entry in stats["top_5_hosts"].items():
        new_hosts.append(entry)

    for ip,entry in stats["top_5_ips"].items():
        new_ips.append(entry)

    new_hosts.sort(key=lambda x:x != None and x.get("count"),reverse=True)
    new_requests.sort(key=lambda x:x != None and x.get("count"),reverse=True)
    new_ips.sort(key=lambda x:x != None and x.get("count"),reverse=True)

    stats["top_5_requests"] = new_requests[:5]
    stats["top_5_hosts"] = new_hosts[:5]
    stats["top_5_ips"] = new_ips[:5]

    top_5_hosts_output = ""
    top_5_requests_output = ""
    top_5_ips_output = ""
    for item in stats["top_5_hosts"]:
        top_5_hosts_output += f"- {item['host_text']} ~ {format(item['count'],',d')} \n".replace('"','')

    for item in stats["top_5_requests"]:
        top_5_requests_output += f"- {item['request_text']} ~ {format(item['count'],',d')} \n".replace('"','')

    for item in stats["top_5_ips"]:
        if item["ip_text"] != "-":
            top_5_ips_output += f"- {item['ip_text']} ~ {format(item['count'],',d')} \n".replace('"','')

    print(f"""
===~ LOG SELECTION STATS ~===
Total Requests: {format (stats['request_count'], ',d')}
Requests Per Min: {round(stats['average_requests_per_minute'],2)}
Average Body Transfer Speed: {round(stats['average_body_byte_speed']/1024/1024,2)} MB/S
Total Data Transfered: {format_file_size(stats['total_data_transfered'])}

Top 5 Requests:
{top_5_requests_output}
Top 5 Hosts:
{top_5_hosts_output}
Top 5 IP Addresses:
{top_5_ips_output}
""") */
